#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct User {
    pub name: String,
    pub user_add: Pubkey,
    pub ticket_count: u64,
    pub balance: u64,
}

impl<'info, 'entrypoint> User {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedUser<'info, 'entrypoint>> {
        let name = account.name.clone();
        let user_add = account.user_add.clone();
        let ticket_count = account.ticket_count;
        let balance = account.balance;

        Mutable::new(LoadedUser {
            __account__: account,
            __programs__: programs_map,
            name,
            user_add,
            ticket_count,
            balance,
        })
    }

    pub fn store(loaded: Mutable<LoadedUser>) {
        let mut loaded = loaded.borrow_mut();
        let name = loaded.name.clone();

        loaded.__account__.name = name;

        let user_add = loaded.user_add.clone();

        loaded.__account__.user_add = user_add;

        let ticket_count = loaded.ticket_count;

        loaded.__account__.ticket_count = ticket_count;

        let balance = loaded.balance;

        loaded.__account__.balance = balance;
    }
}

#[derive(Debug)]
pub struct LoadedUser<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, User>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub name: String,
    pub user_add: Pubkey,
    pub ticket_count: u64,
    pub balance: u64,
}

#[account]
#[derive(Debug)]
pub struct Manager {
    pub name: String,
    pub manager_add: Pubkey,
    pub ticket_price: u64,
    pub ticket_count: u64,
    pub winner_no: u64,
    pub winner_address: Pubkey,
}

impl<'info, 'entrypoint> Manager {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedManager<'info, 'entrypoint>> {
        let name = account.name.clone();
        let manager_add = account.manager_add.clone();
        let ticket_price = account.ticket_price;
        let ticket_count = account.ticket_count;
        let winner_no = account.winner_no;
        let winner_address = account.winner_address.clone();

        Mutable::new(LoadedManager {
            __account__: account,
            __programs__: programs_map,
            name,
            manager_add,
            ticket_price,
            ticket_count,
            winner_no,
            winner_address,
        })
    }

    pub fn store(loaded: Mutable<LoadedManager>) {
        let mut loaded = loaded.borrow_mut();
        let name = loaded.name.clone();

        loaded.__account__.name = name;

        let manager_add = loaded.manager_add.clone();

        loaded.__account__.manager_add = manager_add;

        let ticket_price = loaded.ticket_price;

        loaded.__account__.ticket_price = ticket_price;

        let ticket_count = loaded.ticket_count;

        loaded.__account__.ticket_count = ticket_count;

        let winner_no = loaded.winner_no;

        loaded.__account__.winner_no = winner_no;

        let winner_address = loaded.winner_address.clone();

        loaded.__account__.winner_address = winner_address;
    }
}

#[derive(Debug)]
pub struct LoadedManager<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Manager>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub name: String,
    pub manager_add: Pubkey,
    pub ticket_price: u64,
    pub ticket_count: u64,
    pub winner_no: u64,
    pub winner_address: Pubkey,
}

pub fn use_token_mint_handler<'info>(
    mut mint: SeahorseAccount<'info, '_, Mint>,
    mut recipient: SeahorseAccount<'info, '_, TokenAccount>,
    mut signer: SeahorseSigner<'info, '_>,
    mut manager: Mutable<LoadedManager<'info, '_>>,
) -> () {
    if !(signer.key() == manager.borrow().manager_add) {
        panic!("Only Manager is authorised to this function");
    }

    token::mint_to(
        CpiContext::new(
            mint.programs.get("token_program"),
            token::MintTo {
                mint: mint.to_account_info(),
                authority: signer.to_account_info(),
                to: recipient.to_account_info(),
            },
        ),
        10000,
    )
    .unwrap();
}

pub fn manager_init_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut name: String,
    mut manager: Empty<Mutable<LoadedManager<'info, '_>>>,
    mut winner_random_no: u64,
) -> () {
    let mut manager = manager.account.clone();

    assign!(manager.borrow_mut().manager_add, owner.key());

    assign!(manager.borrow_mut().name, name);

    assign!(manager.borrow_mut().ticket_price, 5);

    assign!(manager.borrow_mut().ticket_count, 0);

    assign!(manager.borrow_mut().winner_no, winner_random_no);
}

pub fn init_users_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut user: Empty<Mutable<LoadedUser<'info, '_>>>,
    mut name: String,
    mut manager: Mutable<LoadedManager<'info, '_>>,
) -> () {
    let mut user = user.account.clone();

    assign!(user.borrow_mut().name, name);

    assign!(user.borrow_mut().user_add, owner.key());

    assign!(user.borrow_mut().balance, 20);
}

pub fn buy_tickets_handler<'info>(
    mut user: Mutable<LoadedUser<'info, '_>>,
    mut manager: Mutable<LoadedManager<'info, '_>>,
    mut signer: SeahorseSigner<'info, '_>,
    mut user_token: Empty<SeahorseAccount<'info, '_, TokenAccount>>,
    mut mint: SeahorseAccount<'info, '_, Mint>,
) -> () {
    user_token.account.clone();

    assign!(
        user.borrow_mut().balance,
        user.borrow().balance - manager.borrow().ticket_price
    );

    assign!(
        user.borrow_mut().ticket_count,
        user.borrow().ticket_count + 1
    );

    assign!(
        manager.borrow_mut().ticket_count,
        manager.borrow().ticket_count + 1
    );

    if manager.borrow().ticket_count == manager.borrow().winner_no {
        assign!(manager.borrow_mut().winner_address, user.borrow().user_add);
    }
}

pub fn manager_token_acc_handler<'info>(
    mut manager_token_acc: Empty<SeahorseAccount<'info, '_, TokenAccount>>,
    mut mint: SeahorseAccount<'info, '_, Mint>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    manager_token_acc.account.clone();
}

pub fn init_token_mint_handler<'info>(
    mut new_token_mint: Empty<SeahorseAccount<'info, '_, Mint>>,
    mut signer: SeahorseSigner<'info, '_>,
    mut manager: Mutable<LoadedManager<'info, '_>>,
) -> () {
    if !(signer.key() == manager.borrow().manager_add) {
        panic!("Only Manager is authorised to this function");
    }

    new_token_mint.account.clone();
}

pub fn check_winner_handler<'info>(
    mut user: Mutable<LoadedUser<'info, '_>>,
    mut manager: Mutable<LoadedManager<'info, '_>>,
    mut user_token: SeahorseAccount<'info, '_, TokenAccount>,
    mut manager_token: SeahorseAccount<'info, '_, TokenAccount>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    if !(signer.key() == manager.borrow().manager_add) {
        panic!("Manager is authorised to it");
    }

    if user.borrow().user_add == manager.borrow().winner_address {
        solana_program::msg!("{}", "Congrats you have won the lottery!!");

        token::transfer(
            CpiContext::new(
                manager_token.programs.get("token_program"),
                token::Transfer {
                    from: manager_token.to_account_info(),
                    authority: signer.to_account_info(),
                    to: user_token.to_account_info(),
                },
            ),
            9000,
        )
        .unwrap();

        assign!(user.borrow_mut().balance, user.borrow().balance + 9000);
    } else {
        solana_program::msg!("{}", "Sorry, Try next time.");
    }
}
