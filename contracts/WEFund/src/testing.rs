use super::*;
use cosmwasm_std::{from_binary, Addr, CosmosMsg, WasmMsg,
    BankQuery, BalanceResponse, AllBalanceResponse, Coin, Uint128};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR, mock_dependencies};

use crate::contract::{execute, instantiate};
use crate::query::{query};
use crate::state::{Milestone, Config, ProjectState};
use crate::msg::{QueryMsg, ExecuteMsg, InstantiateMsg};

// use crate::mock_querier::mock_dependencies;
use cw20::Cw20ExecuteMsg;
// use terraswap::asset::{Asset, AssetInfo};
// use terraswap::pair::ExecuteMsg as TerraswapExecuteMsg;

#[test]
fn workflow(){
    let mut deps = mock_dependencies(&[]);
    
    let msg = InstantiateMsg{
        admin: Some(String::from("admin")),
        wefund: Some(String::from("Wefund")),
        anchor_market: Some( "market".to_string()),
        aust_token: Some("aust".to_string()),
        fundraising_contract: Some("fundrasing".to_string()),
        vesting_contract: Some("vesting".to_string())
    };
//instantiate
    let info = mock_info("admin", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
//add community member
    let msg = ExecuteMsg::AddCommunitymember{
        wallet: String::from("community1")
    };
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Add community member{:?}", res);
    //-------------------------------
    let msg = ExecuteMsg::AddCommunitymember{
        wallet: String::from("community2")
    };
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Add community member{:?}", res);

    // let msg = ExecuteMsg::RemoveCommunitymember{
    //     wallet: String::from("community3")
    // };
    // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    // println!("Remove community member{:?}", res);
//add project        
    let milestone1 = Milestone{
        milestone_step: Uint128::new(0),
        milestone_name: String::from("milestone1"),
        milestone_description: String::from("mileston1"),
        milestone_startdate: String::from("startdate"),
        milestone_enddate: String::from("enddate"),
        milestone_amount: Uint128::new(100),
        milestone_status: Uint128::new(0),
        milestone_votes: Vec::new()
    };
    let milestone2 = Milestone{
        milestone_step: Uint128::new(1),
        milestone_name: String::from("milestone2"),
        milestone_description: String::from("mileston2"),
        milestone_startdate: String::from("startdate"),
        milestone_enddate: String::from("enddate"),
        milestone_amount: Uint128::new(200),
        milestone_status: Uint128::new(0),
        milestone_votes: Vec::new()
    };
    let milestone_states = vec![milestone1, milestone2];
    let msg = ExecuteMsg::AddProject{
        creator_wallet: String::from("terra1emwyg68n0wtglz8ex2n2728fnfzca9xkdc4aka"),
        project_description: String::from("demo1"),
        project_collected: Uint128::new(300),
        project_email: String::from("deme1@gmail.com"),
        project_title: String::from("demo1"),
        project_website: String::from("https://demo1"),
        project_createddate: String::from("20211223"),
        project_logo: String::from("icon1"),
        project_whitepaper: String::from("whitepaper"),
        project_milestones: milestone_states,
        project_company: "company".to_string(),
        project_ecosystem: "terra".to_string(),
        project_saft: "saft".to_string(),
        project_teammembers: Vec::new(),
        vesting: Vec::new(),
        token_addr: "token1".to_string(),
    };
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    // assert_eq!(res.messages.len(), 0);
    println!("{:?}", res);

//add project        

//     let msg = ExecuteMsg::AddProject{
//         creator_wallet: String::from("anyone"),
//         project_description: String::from("demo2"),
//         project_category: String::from("terra"),
//         project_collected: Uint128::new(300),
//         project_chain: String::from("Terra"),
//         project_email: String::from("deme2@gmail.com"),
//         project_name: String::from("demo2"),
//         project_website: String::from("https://demo1"),
//         project_createddate: String::from("20211223"),
//         project_icon: String::from("icon2"),
//         project_deadline: String::from("20220130"),
//         project_subcategory: String::from("gaming"),
//         project_teamdescription: String::from("demo2"),
//         project_whitepaper: String::from("whitepaper"),
//         project_milestones: Vec::new(),
//     };
//     let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
//     // assert_eq!(res.messages.len(), 0);
//     println!("{:?}", res);

// //Wefund Approve
        let info = mock_info("admin", &[]);
        let msg = ExecuteMsg::WefundApprove{
            project_id: Uint128::new(1),
            deadline: Uint128::from(mock_env().block.time.seconds())
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("WeFund Approve: {:?}", res);

        // let info = mock_info("admin", &[]);
        // let msg = ExecuteMsg::WefundApprove{
        //     project_id: Uint128::new(2),
        //     deadline: Uint128::from(mock_env().block.time.seconds())
        // };
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        // println!("WeFund Approve: {:?}", res);
// //Set community vote
        let info = mock_info("community1", &[]);
        let msg = ExecuteMsg::SetCommunityVote{
            project_id: Uint128::new(1),
            wallet: String::from("community1"),
            voted: true
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("Set Community vote: {:?}", res);

        // let info = mock_info("community2", &[]);
        // let msg = ExecuteMsg::SetCommunityVote{
        //     project_id: Uint128::new(1),
        //     wallet: String::from("community2"),
        //     voted: true
        // };
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        // println!("Set Community vote: {:?}", res);
// // back 2 projct
        let info = mock_info("backer1", &[Coin::new(105000000, "uusd")]);
        let msg = ExecuteMsg::Back2Project{
            project_id: Uint128::new(1),
            backer_wallet: String::from("backer1"),
            otherchain: "ethereum".to_string(),
            otherchain_wallet: "ether_wallet".to_string(),
            fundraising_stage: "seed".to_string(),
            token_amount: Uint128::new(10)
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("back2project:{:?}", res);

        let info = mock_info("backer2", &[Coin::new(210000000, "uusd")]);
        let msg = ExecuteMsg::Back2Project{
            project_id: Uint128::new(1),
            backer_wallet: String::from("backer2"),
            otherchain: "ethereum".to_string(),
            otherchain_wallet: "ether_wallet".to_string(),
            fundraising_stage: "seed".to_string(),
            token_amount: Uint128::new(10)
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("back2project:{:?}", res);

        let info = mock_info("community1", &[Coin::new(210000000, "uusd")]);
        let msg = ExecuteMsg::Back2Project{
            project_id: Uint128::new(1),
            backer_wallet: String::from("community1"),
            otherchain: "ethereum".to_string(),
            otherchain_wallet: "ether_wallet".to_string(),
            fundraising_stage: "presale".to_string(),
            token_amount: Uint128::new(10)
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("back2project:{:?}", res);
// //-Get Project-----------------
// let msg = QueryMsg::GetAllProject{};
// let allproject = query(deps.as_ref(), mock_env(), msg).unwrap();

// let res:Vec<ProjectState> = from_binary(&allproject).unwrap();
// println!("allproject {:?}", res );    
// //set milestone vote
        let info = mock_info("backer1", &[]);
        let msg = ExecuteMsg::SetMilestoneVote{
            project_id: Uint128::new(1),
            wallet: String::from("backer1"),
            voted: true,
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("set milestone vote:{:?}", res);

        let info = mock_info("backer2", &[]);
        let msg = ExecuteMsg::SetMilestoneVote{
            project_id: Uint128::new(1),
            wallet: String::from("backer2"),
            voted: true,
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("set milestone vote:{:?}", res);

        let info = mock_info("backer1", &[]);
        let msg = ExecuteMsg::SetMilestoneVote{
            project_id: Uint128::new(1),
            wallet: String::from("backer1"),
            voted: true,
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("set milestone vote:{:?}", res);

        let info = mock_info("backer2", &[]);
        let msg = ExecuteMsg::SetMilestoneVote{
            project_id: Uint128::new(1),
            wallet: String::from("backer2"),
            voted: true,
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("set milestone vote:{:?}", res);
        
// //-Get Project-----------------
//     let msg = QueryMsg::GetAllProject{};
//     let allproject = query(deps.as_ref(), mock_env(), msg).unwrap();

//     let res:Vec<ProjectState> = from_binary(&allproject).unwrap();
//     println!("allproject {:?}", res );
// //-Get Config-------------            
//     let msg = QueryMsg::GetConfig{};
//     let res = query(deps.as_ref(), mock_env(), msg).unwrap();

//     let config:Config= from_binary(&res).unwrap();
//     println!("Config = {:?}", config);
// //-Complete project--------------------------
//     // let msg = ExecuteMsg::CompleteProject{project_id:Uint128::new(1)};
//     // let res = execute(deps.as_mut(), mock_env(), info, msg);

// //-Get project1 Balance-------------------
//     // let msg = QueryMsg::GetBalance{ wallet: String::from("wefund")};
//     // let balance = query(deps.as_ref(), mock_env(), msg).unwrap();

//     // let res:AllBalanceResponse = from_binary(&balance).unwrap();
//     // println!("wefund Balance {:?}", res );
// //-Get wefund Balance-------------------
//     // let msg = QueryMsg::GetBalance{ wallet: String::from("market")};
//     // let balance = query(deps.as_ref(), mock_env(), msg).unwrap();

//     // let res:AllBalanceResponse = from_binary(&balance).unwrap();
//     // println!("market Balance {:?}", res );

//-Remove Project-------------------------
    // let info = mock_info("admin", &[Coin::new(105000000, "uusd")]);
    // let msg = ExecuteMsg::RemoveProject{project_id:Uint128::new(1)};
    // let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//-Get Project-----------------
    let msg = QueryMsg::GetAllProject{};
    let allproject = query(deps.as_ref(), mock_env(), msg).unwrap();

    let res:Vec<ProjectState> = from_binary(&allproject).unwrap();
    println!("allproject {:?}", res );
}

