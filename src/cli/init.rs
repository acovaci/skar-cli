use std::io::{BufWriter, Write};

use skar::core::error::{Res, SkarError};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufStream},
};

const BEGIN_CONFIG: &str = "#[BEGIN SKAR_CONFIG]";
const END_CONFIG: &str = "#[END SKAR_CONFIG]";

const ALIASES: [(&str, &str); 4] = [
    ("?", "shell complete"),
    ("??", "shell explain"),
    ("?!", "shell generate"),
    ("?-", "chat"),
];

pub async fn run() -> Res<()> {
    init_zsh().await?;
    Ok(())
}

pub async fn init_zsh() -> Res<()> {
    let dir = std::env::var("ZDOTDIR")?;
    let dir = std::path::PathBuf::from(dir);
    let zshrc = dir.join(".zshrc");

    let mut config_content = get_config_text()?;

    let file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&zshrc)
        .await?;

    log::debug!("Writing to zshrc: {:?}", file);

    let mut buf = BufStream::new(file);

    let config_pos = seek_to_skar_config(&mut buf).await.map_err(|e| match e {
        SkarError::_Internal(_) => {
            SkarError::InvalidShellConfig(zshrc, "Found end of config before beginning".to_string())
        }
        _ => e,
    })?;

    let mut content = Vec::new();
    buf.read_to_end(&mut content).await?;
    config_content.write_all(&content)?;

    buf.seek(tokio::io::SeekFrom::Start(config_pos.0 as u64))
        .await?;

    let content = config_content
        .into_inner()
        .map_err(|e| SkarError::IOError(e.into_error()))?;
    buf.write_all(&content).await?;

    let pos = buf.stream_position().await?;

    buf.flush().await?;

    buf.get_mut().set_len(pos).await?;

    Ok(())
}

pub async fn seek_to_skar_config(buf: &mut BufStream<File>) -> Res<(usize, usize)> {
    let mut line = String::new();
    let mut begin = None;
    let mut previous_pos = 0;

    buf.seek(tokio::io::SeekFrom::Start(0)).await?;

    while buf.read_line(&mut line).await? > 0 {
        if line.contains(BEGIN_CONFIG) && begin.is_none() {
            begin = Some(previous_pos);
            log::debug!("Found existing skar init at position {}", begin.unwrap());
        }

        if line.contains(END_CONFIG) {
            let begin = begin.ok_or(SkarError::_Internal("end_config".to_string()))?;

            let end = buf.stream_position().await?;
            log::debug!("Found end of existing skar init at position {}", end);

            log::debug!("Returning begin: {}, end: {}", begin, end);
            return Ok((begin, end as usize));
        }

        previous_pos = buf.stream_position().await? as usize;
    }

    Ok((previous_pos, previous_pos))
}

pub fn get_config_text() -> Res<BufWriter<Vec<u8>>> {
    let mut buf = BufWriter::new(Vec::new());

    let crate_dir = std::env::current_exe()?;

    buf.write_all(BEGIN_CONFIG.as_bytes())?;
    buf.write_all(&[b'\n'])?;

    for (alias, command) in ALIASES.iter() {
        buf.write_all(
            format!(
                "alias '{}'='{} {}'",
                alias,
                crate_dir.display().to_string(),
                command
            )
            .as_bytes(),
        )?;
        buf.write_all(&[b'\n'])?;
    }

    buf.write_all(END_CONFIG.as_bytes())?;

    Ok(buf)
}
