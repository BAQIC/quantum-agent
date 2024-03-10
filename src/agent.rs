use reqwest::Response;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum AgentType {
    #[serde(rename = "qpp-sv")]
    QppSV,
    #[serde(rename = "qpp-dm")]
    QppDM,
    #[serde(rename = "qasmsim")]
    QASMSim,
    #[serde(rename = "cudaq")]
    CUDAQ,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentAddress {
    pub cudaq_agent: String,
    pub qpp_agent: String,
    pub qasmsim_agent: String,
}

// read agent.yaml file, and return map
pub fn read_config() -> AgentAddress {
    let f = std::fs::File::open("agent.yaml").expect("Can not open file");
    let agent_config: AgentAddress = serde_yaml::from_reader(f).expect("Can not read values");
    agent_config
}

async fn submit_qpp(
    address: &str,
    code: &str,
    shots: usize,
    backend: &str,
) -> Result<Response, reqwest::Error> {
    let body = [
        ("qasm", code.to_string()),
        ("shots", shots.to_string()),
        ("backend", backend.to_string()),
    ];

    reqwest::Client::new()
        .post(address)
        .form(&body)
        .send()
        .await
}

async fn submit_qasmsim(
    address: &str,
    code: &str,
    shots: usize,
) -> Result<Response, reqwest::Error> {
    let body = [("qasm", code.to_string()), ("shots", shots.to_string())];
    reqwest::Client::new()
        .post(address)
        .form(&body)
        .send()
        .await
}

pub async fn run(
    code: &str,
    shots: usize,
    agent: AgentType,
    agent_address: &AgentAddress,
) -> Result<Response, reqwest::Error> {
    match agent {
        AgentType::QppSV => submit_qpp(&agent_address.qpp_agent, code, shots, "sv").await,
        AgentType::QppDM => submit_qpp(&agent_address.qpp_agent, code, shots, "dm").await,
        AgentType::QASMSim => submit_qasmsim(&agent_address.qasmsim_agent, code, shots).await,
        AgentType::CUDAQ => todo!(),
    }
}
