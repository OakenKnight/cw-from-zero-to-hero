#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

use self::exec::{create_poll, vote};


const CONTRACT_NAME: &str = "crates.io:cw-from-zero-to-hero";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or(info.sender.to_string());

    let validate_admin = deps.api.addr_validate(&admin)?;
    let config = Config{ 
        admin: validate_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validate_admin.to_string()))

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll { poll_id, question, options } => exec::create_poll(deps, env,info, poll_id, question, options),
        ExecuteMsg::Vote { poll_id, vote } => exec::vote(deps, env, info, poll_id, vote)
    }
}
mod exec {
    use cosmwasm_std::{Response, DepsMut, Env, MessageInfo, StdResult};
    use crate::state::{Config, CONFIG, Poll, POLLS, Ballot, BALLOTS};
    use crate::ContractError;

    pub fn create_poll(
        deps: DepsMut, 
        env: Env, 
        info: MessageInfo, 
        poll_id: String, 
        question: String, 
        options: Vec<String>) -> Result<Response, ContractError>{
        if options.len() > 10 { return Err(ContractError::TooManyOptions {  })}
        let mut opts: Vec<(String, u64)> = vec![];

        for option in options {
            opts.push((option, 0));
        }
        let poll = Poll {
            creator: info.sender,
            question: question,
            options: opts
        };
        POLLS.save(deps.storage, poll_id.clone(), &poll)?;

        Ok(Response::new()
            .add_attribute("action", "create_poll")
            .add_attribute("poll_id", poll_id)) 
       }
    pub fn vote(
        deps: DepsMut, 
        env: Env, 
        info: MessageInfo,
        poll_id: String, vote: String) -> Result<Response, ContractError>{
        let poll = POLLS.may_load(deps.storage, poll_id.clone())?;
        

        // either no poll or update the current one
        match poll{
            // if found poll, update current vote option (--), vote for another option (++)
            Some(mut poll) => {
                BALLOTS.update(
                    deps.storage,
                    (info.sender, poll_id.clone()),
                    |ballot| -> StdResult<Ballot>{
                        match ballot {
                            Some(ballot) => {
                                let position_of_old_vote = poll
                                    .options
                                    .iter()
                                    .position(|p| p.0 == ballot.option)
                                    .unwrap();
                                    poll.options[position_of_old_vote].1 -=1;
                                Ok( Ballot{option: vote.clone() })
                            },
                            None => {
                                Ok( Ballot { option: vote.clone() })
                            }
                        }
                    }
                )?;
                

                let position = poll
                    .options
                    .iter()
                    .position(|p| p.0 == vote.clone());
                if position.is_none() {
                    return Err(ContractError::VoteOptionNotFound {})
                }   
                    let position = position.unwrap();
                    
                    poll.options[position].1 +=1; 
                    POLLS.save(deps.storage, poll_id.clone(), &poll)?;
                    
                    return Ok(Response::new()
                        .add_attribute("action", "vote")
                        .add_attribute("poll_id", poll_id)
                        .add_attribute(vote.to_string(), poll.options[position].1.to_string()))   
                
            },
            None => return Err(ContractError::Unauthorized),
            }
                 
        }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::attr; // helper to construct an attribute e.g. ("action", "instantiate")
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info}; use crate::ContractError;
    // mock functions to mock an environment, message info, dependencies
    use crate::contract::{instantiate, execute}; // the contract instantiate function
    use crate::msg::{InstantiateMsg, ExecuteMsg}; // our instantate method

    // Two fake addresses we will use to mock_info
    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin : None};

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        ) 
   }

   #[test]
   fn test_instantiate_with_admin(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin : Some(ADDR2.to_string())};

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR2)]
        ) 
   }

   #[test]
   fn test_execute_create_poll_valid(){
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "create_poll"), attr("poll_id", "some_id")]
    )
   }

   #[test]
    fn test_execute_create_poll_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite number?".to_string(),
            options: vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
                "11".to_string(),
            ],
        };
    
        // Unwrap error to assert failure
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(err, ContractError::TooManyOptions {  })
    }
    #[test]
    fn test_execute_vote_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the vote, first time voting
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Change the vote
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Osmosis".to_string(),
        };
        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "vote"), attr("poll_id", "some_id"),attr("Osmosis", "1")]
        )
       
    }
    
    #[test]
    fn test_execute_vote_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the vote, some_id poll is not created yet.
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };
        // Unwrap to assert error
        let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        // Create the poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Vote on a now existing poll but the option "DVPN" does not exist
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "DVPN".to_string(),
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(err, ContractError::VoteOptionNotFound{})
    }
}
