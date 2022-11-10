#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod seahorse_util {
    use super::*;
    use std::{collections::HashMap, fmt::Debug, ops::Deref};

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: Clone> Mutable<Vec<T>> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index > 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    impl<T: Clone, const N: usize> Mutable<[T; N]> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index > 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }
}

#[program]
mod lottery_program {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    pub struct UseTokenMint<'info> {
        #[account(mut)]
        pub mint: Box<Account<'info, Mint>>,
        #[account(mut)]
        pub recipient: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        #[account(mut)]
        pub manager: Box<Account<'info, dot::program::Manager>>,
        pub token_program: Program<'info, Token>,
    }

    pub fn use_token_mint(ctx: Context<UseTokenMint>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let mint = SeahorseAccount {
            account: &ctx.accounts.mint,
            programs: &programs_map,
        };

        let recipient = SeahorseAccount {
            account: &ctx.accounts.recipient,
            programs: &programs_map,
        };

        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        let manager = dot::program::Manager::load(&mut ctx.accounts.manager, &programs_map);

        use_token_mint_handler(
            mint.clone(),
            recipient.clone(),
            signer.clone(),
            manager.clone(),
        );

        dot::program::Manager::store(manager);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (name : String , winner_random_no : u64)]
    pub struct ManagerInit<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Manager > () + 8 , payer = owner , seeds = ["manager" . as_bytes () . as_ref () , owner . key () . as_ref ()] , bump)]
        pub manager: Box<Account<'info, dot::program::Manager>>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }

    pub fn manager_init(
        ctx: Context<ManagerInit>,
        name: String,
        winner_random_no: u64,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let manager = Empty {
            account: dot::program::Manager::load(&mut ctx.accounts.manager, &programs_map),
            bump: ctx.bumps.get("manager").map(|bump| *bump),
        };

        manager_init_handler(owner.clone(), name, manager.clone(), winner_random_no);

        dot::program::Manager::store(manager.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (name : String)]
    pub struct InitUsers<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: User > () + 8 , payer = owner , seeds = ["user" . as_bytes () . as_ref () , owner . key () . as_ref ()] , bump)]
        pub user: Box<Account<'info, dot::program::User>>,
        #[account(mut)]
        pub manager: Box<Account<'info, dot::program::Manager>>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }

    pub fn init_users(ctx: Context<InitUsers>, name: String) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let user = Empty {
            account: dot::program::User::load(&mut ctx.accounts.user, &programs_map),
            bump: ctx.bumps.get("user").map(|bump| *bump),
        };

        let manager = dot::program::Manager::load(&mut ctx.accounts.manager, &programs_map);

        init_users_handler(owner.clone(), user.clone(), name, manager.clone());

        dot::program::User::store(user.account);

        dot::program::Manager::store(manager);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct BuyTickets<'info> {
        #[account(mut)]
        pub user: Box<Account<'info, dot::program::User>>,
        #[account(mut)]
        pub manager: Box<Account<'info, dot::program::Manager>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        # [account (init , payer = signer , seeds = ["Token" . as_bytes () . as_ref () , signer . key () . as_ref ()] , bump , token :: mint = mint , token :: authority = signer)]
        pub user_token: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub mint: Box<Account<'info, Mint>>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
        pub token_program: Program<'info, Token>,
    }

    pub fn buy_tickets(ctx: Context<BuyTickets>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let user = dot::program::User::load(&mut ctx.accounts.user, &programs_map);
        let manager = dot::program::Manager::load(&mut ctx.accounts.manager, &programs_map);
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        let user_token = Empty {
            account: SeahorseAccount {
                account: &ctx.accounts.user_token,
                programs: &programs_map,
            },
            bump: ctx.bumps.get("user_token").map(|bump| *bump),
        };

        let mint = SeahorseAccount {
            account: &ctx.accounts.mint,
            programs: &programs_map,
        };

        buy_tickets_handler(
            user.clone(),
            manager.clone(),
            signer.clone(),
            user_token.clone(),
            mint.clone(),
        );

        dot::program::User::store(user);

        dot::program::Manager::store(manager);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct ManagerTokenAcc<'info> {
        # [account (init , payer = signer , seeds = ["token" . as_bytes () . as_ref () , signer . key () . as_ref ()] , bump , token :: mint = mint , token :: authority = signer)]
        pub manager_token_acc: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub mint: Box<Account<'info, Mint>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub token_program: Program<'info, Token>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn manager_token_acc(ctx: Context<ManagerTokenAcc>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let manager_token_acc = Empty {
            account: SeahorseAccount {
                account: &ctx.accounts.manager_token_acc,
                programs: &programs_map,
            },
            bump: ctx.bumps.get("manager_token_acc").map(|bump| *bump),
        };

        let mint = SeahorseAccount {
            account: &ctx.accounts.mint,
            programs: &programs_map,
        };

        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        manager_token_acc_handler(manager_token_acc.clone(), mint.clone(), signer.clone());

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct InitTokenMint<'info> {
        # [account (init , payer = signer , seeds = ["token-mint" . as_bytes () . as_ref () , signer . key () . as_ref ()] , bump , mint :: decimals = 0 , mint :: authority = signer)]
        pub new_token_mint: Box<Account<'info, Mint>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        #[account(mut)]
        pub manager: Box<Account<'info, dot::program::Manager>>,
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
        pub rent: Sysvar<'info, Rent>,
    }

    pub fn init_token_mint(ctx: Context<InitTokenMint>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let new_token_mint = Empty {
            account: SeahorseAccount {
                account: &ctx.accounts.new_token_mint,
                programs: &programs_map,
            },
            bump: ctx.bumps.get("new_token_mint").map(|bump| *bump),
        };

        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        let manager = dot::program::Manager::load(&mut ctx.accounts.manager, &programs_map);

        init_token_mint_handler(new_token_mint.clone(), signer.clone(), manager.clone());

        dot::program::Manager::store(manager);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct CheckWinner<'info> {
        #[account(mut)]
        pub user: Box<Account<'info, dot::program::User>>,
        #[account(mut)]
        pub manager: Box<Account<'info, dot::program::Manager>>,
        #[account(mut)]
        pub user_token: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub manager_token: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub token_program: Program<'info, Token>,
    }

    pub fn check_winner(ctx: Context<CheckWinner>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let user = dot::program::User::load(&mut ctx.accounts.user, &programs_map);
        let manager = dot::program::Manager::load(&mut ctx.accounts.manager, &programs_map);
        let user_token = SeahorseAccount {
            account: &ctx.accounts.user_token,
            programs: &programs_map,
        };

        let manager_token = SeahorseAccount {
            account: &ctx.accounts.manager_token,
            programs: &programs_map,
        };

        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        check_winner_handler(
            user.clone(),
            manager.clone(),
            user_token.clone(),
            manager_token.clone(),
            signer.clone(),
        );

        dot::program::User::store(user);

        dot::program::Manager::store(manager);

        return Ok(());
    }
}
