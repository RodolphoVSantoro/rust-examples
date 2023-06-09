use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::time::Instant;

fn main() -> io::Result<()> {
    let start = Instant::now();
    let file = File::open("../ceps-number-sorted.txt")?;
    let mut reader = BufReader::new(file);
    // let mut writer = File::create("cep.txt")?;
    let mut bin_writer = BufWriter::new(File::create("cep.bin")?);
    let mut i = 0;

    // let parsed_ceps: Vec<CEP> = serde_json::from_reader(reader).unwrap();
    // let cep_list = read_to_string("../ceps-number-sorted.txt").unwrap();

    let mut line = String::new();
    let mut result = reader.read_line(&mut line).unwrap();
    while result != 0 {
        let cep_number = &line[..8]
            .parse::<u32>()
            .expect(format!("Erro ao converter o CEP {}", line).as_str());
        // writer.write_all((cep + "\n").as_bytes())?;
        bin_writer.write_all(&cep_number.to_le_bytes())?;
        line.clear();
        result = reader.read_line(&mut line).unwrap();
        i += 1;
    }
    // writer.flush()?;
    bin_writer.flush()?;

    let duration = start.elapsed();
    dbg!("{} linhas lidas em {} segundos", i, duration);
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CEP {
    uf: String,
    cep: String,
    localidade: String,

    #[serde(rename = "locNoSem")]
    loc_no_sem: String,

    #[serde(rename = "locNu")]
    loc_nu: String,

    #[serde(rename = "localidadeSubordinada")]
    localidade_subordinada: String,

    #[serde(rename = "logradouroDNEC")]
    logradouro_dnec: String,

    #[serde(rename = "logradouroTextoAdicional")]
    logradouro_texto_adicional: String,

    #[serde(rename = "logradouroTexto")]
    logradouro_texto: String,

    bairro: String,

    #[serde(rename = "baiNu")]
    bai_nu: String,

    #[serde(rename = "nomeUnidade")]
    nome_unidade: String,

    #[serde(rename = "tipoCep")]
    tipo_cep: String,

    #[serde(rename = "numeroLocalidade")]
    numero_localidade: String,

    situacao: String,

    #[serde(rename = "faixasCaixaPostal")]
    faixas_caixa_postal: Vec<Option<serde_json::Value>>,

    #[serde(rename = "faixasCep")]
    faixas_cep: Vec<Option<serde_json::Value>>,
}
