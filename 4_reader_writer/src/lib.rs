use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub const BUFFER_SIZE: usize = 64 * 1024;

// -----------------------------------------------------------------------------
// MyBufReader
// -----------------------------------------------------------------------------

#[allow(dead_code)]
pub struct MyBufReader {
    // TODO: добавьте необходимые поля для буферизации чтения
    // возможно, вам понадобится что-то вроде:
    // поле для хранения данных, прочитанных из файла, но ещё не отданных пользователю,
    // или позиция внутри этого буфера, откуда отдавать следующий байт.
    // ну и возможно что-то ещё, что поможет вам реализовать read_byte() эффективно
    f: File,
    b: [u8; BUFFER_SIZE],
    /// прочитано байт
    l: usize,
    /// индекс внутри буффера. Может равняться или быть больше l
    bi: usize
}

impl MyBufReader {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let f = File::open(path)?;
        Ok(Self {
            f,
            b: [0; BUFFER_SIZE],
            l: 0,
            bi: 0
        })
    }

    pub fn read_byte(&mut self) -> io::Result<Option<u8>> {
        if self.bi >= self.l {
            self.l = self.f.read(&mut self.b)?;
            self.bi = 0;
        }
        if self.l == 0 {
            return Ok(None)
        }
        let res = self.b[self.bi];
        self.bi += 1;
        Ok(Some(res))
    }
}

// -----------------------------------------------------------------------------
// MyBufWriter
// -----------------------------------------------------------------------------

pub struct MyBufWriter {
    // TODO: добавьте необходимые поля для буферизации записи
    // наверное, вам понадобится буфер для хранения данных, которые пользователь уже передал в
    // write_buffered(),
    // но ещё не записал в файл, и, конечно же, еще какие-то поля для работы с файлом
    f: File,
    b: [u8; BUFFER_SIZE],
    /// Индекс следующего свободного элементы в буфере
    bi: usize
}

impl MyBufWriter {
    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let f = File::create(path)?;
        Ok(Self {
            f,
            b: [0; BUFFER_SIZE],
            bi: 0
        })
    }

    pub fn write_buffered(&mut self, data: &[u8]) -> io::Result<()> {
        let mut n = 0;
        loop {
            if n == data.len() {
                break;
            }

            if self.bi == self.b.len() {
                self.flush()?;
            }

            let copied = (&mut self.b[self.bi..]).write(&data[n..])?;

            self.bi += copied;
            n += copied;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        // TODO: эта функция должна записать в файл все данные, которые сейчас лежат во внутреннем
        // буфере, верно? И после этого внутренний буфер должен быть пустым, готовым для новых
        // данных от пользователя.
        // но пока что просто заглушка, чтобы код компилировался, вам нужно реализовать эту функцию
        let mut written = 0;

        let result = loop {
            if written == self.bi {
                break Ok(());
            }

            match self.f.write(&self.b[written..self.bi]) {
                Ok(0) => {
                    break Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "не удалось продолжить запись",
                    ));
                }
                Ok(n) => written += n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => break Err(e),
            }
        };

        // Сохраняем незаписанные данные в начале буфера.
        self.b.copy_within(written..self.bi, 0);
        self.bi -= written;

        result?;
        self.f.flush()
    }

    pub fn close(mut self) -> io::Result<()> {
        self.flush()
    }
}

impl Drop for MyBufWriter {
    fn drop(&mut self) {
        // Ошибку из Drop вернуть нельзя.
        // Поэтому в реальном коде лучше явно вызывать close() или flush().
        let _ = self.flush();
    }
}

// -----------------------------------------------------------------------------
// Медленная версия
// -----------------------------------------------------------------------------

pub fn copy_slow(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut input = File::open(input)?;
    let mut output = File::create(output)?;

    let mut copied = 0;
    let mut byte = [0u8; 1];

    loop {
        let n = input.read(&mut byte)?;
        if n == 0 {
            break;
        }

        output.write_all(&byte[..n])?;
        copied += n as u64;
    }

    output.flush()?;

    Ok(copied)
}

// -----------------------------------------------------------------------------
// Быстрая версия
// -----------------------------------------------------------------------------
// copy_fast специально тоже использует побайтный API.
// Разница должна быть не в коде копирования, а в реализации MyBufReader и MyBufWriter
// эту функцию не нужно менять, она должна работать с любыми реализациями MyBufReader и MyBufWriter,
// которые вы сделаете
pub fn copy_fast(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut reader = MyBufReader::open(input)?;
    let mut writer = MyBufWriter::create(output)?;

    let mut copied = 0;

    while let Some(byte) = reader.read_byte()? {
        writer.write_buffered(&[byte])?;
        copied += 1;
    }

    writer.close()?;

    Ok(copied)
}

pub const RECORD_SIZE: usize = 10;

pub fn make_record(index: usize) -> [u8; RECORD_SIZE] {
    let mut record = [0u8; RECORD_SIZE];

    (0..RECORD_SIZE).for_each(|i| {
        record[i] = ((index + i) % 251) as u8;
    });

    record
}

pub fn generate_input_file(path: impl AsRef<Path>, records: usize) -> io::Result<()> {
    let mut file = File::create(path)?;

    for i in 0..records {
        let record = make_record(i);
        file.write_all(&record)?;
    }

    file.flush()?;

    Ok(())
}
