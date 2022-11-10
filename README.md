**This project was created by Seahorse 0.2.2.**

# Seahorse Lottery Program
The Objective of this project is to create a **Lottery smart contract** where users can buy lottery tickets and a random winner will be selected from all participants and lottery prize will be rewarded to the winner.

## Prerequisites
We need to install some command line tools for this project to build. We need [Solana](https://docs.solana.com/cli/install-solana-cli-tools), [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html#install-rust), [NodeJS](https://nodejs.org/en/) and [Seahorse](https://seahorse-lang.org/docs/installation). The links provided contain the step-by-step guide on installing these tools and the dependencies required for them like Rust.

You can check if package got installed correctly by running command like :

`solana -V`
`anchor -V`
`seahorse -V`

For this project, the version used are :
* anchor 0.25.0

* seahorse v0.2.2

* node 19.0.0

## Getting started with Seahorse
We initialize Seahorse Project using command `seahorse init Lottery`. This will create a project directory with multiple files which is mostly similar to anchor projects, just will write our seahorse program in `Lottery/programs_py/lottery.py`

## Involved Accounts
There are 2 types of entities involved here, which are **Manager** and **User**. The Manager is responsible for creating the Lottery and its parameters like lottery price and calling the random winner from all lottery buyers. On the other hand, Users are the people who are buying the lottery. 

Manager Account has fields like Name, ` Public address `, ` ticket_price `, `total ticket_count `, ` winner_no` (a random no for deciding winner), ` winner_address `(for storing the winner Public key). User Account has fields like Username, `user address`, ticket_count(number of tickets user bought), `User balance`.

<div>
  
<img src="https://github.com/akshaydhayal/Seahorse-Lottery/blob/master/assets/ManagerDetail.png" alt="Alt text" title="Optional title" height="170" width="300">
  
<img src="https://github.com/akshaydhayal/Seahorse-Lottery/blob/master/assets/UserDetail.png" alt="Alt text" title="Optional title" height="170" width="300">
<!-- <img src="https://github.com/akshaydhayal/Seahorse-Lottery/blob/master/assets/Instructions.png" alt="Alt text" title="Optional title" height="260" width="180">
 -->
</div>

## Program Instructions
We have 7 functions/instructions in this Program. Let's understand all the different Program instructions one by one.

### 1. ManagerInit
This is the function where we **initialise** the Manager's Account. We set the lottery price here(took $5 here) , ticket counter as 0 and the random number(which will be used for winner selection). Winner address is garbage value at start, will be updated at later functions.

```
class Manager(Account):
  name:str
  manager_add:Pubkey
  ticket_price:u64
  ticket_count:u64
  winner_no:u64
  winner_address:Pubkey )
  ```

</div>
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/1.png" alt="Alt text" title="Optional title" height="120" width="380">

### 2. initTokenMint  
Here, we are initialsing the TokenMint. TokenMint are the Accounts that can create SPL tokens. Accounts of this type are owned by the SPL token program. There is assert condition so that only Manager can call this function.
```
@instruction
def init_token_mint(new_token_mint: Empty[TokenMint], signer: Signer, manager:Manager):
  #assert condition so that only manager can call this function
  assert(signer.key()==manager.manager_add),"Only Manager is authorised to this function"  
  new_token_mint.init(
    payer = signer,
    seeds = ['token-mint', signer],
    decimals = 0,
    authority = signer
  )
```
### 3. managerTokenAcc
Initialise a Token Account for Manager Account so that it can hold Mint Tokens.Token Accounts are Account that holds SPL tokens. Accounts of this type are owned by the SPL token program. Now, its just showing 0 Tokens as we have not minted our tokens.

<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/3.png" alt="Alt text" title="Optional title" height="90" width="690">

### 4. useTokenMint
In this function, we are minting **10000 Tokens** to the Manager Token Account. Again, added assert condition, so that no one call this out other than Manager.
```
@instruction
def use_token_mint(mint: TokenMint,recipient: TokenAccount,signer: Signer,manager:Manager):
  # Minting 10000 tokens from our `mint` to `recipient` i.e Manager.

  assert(signer.key()==manager.manager_add),"Only Manager is authorised to this function"
  mint.mint(
    authority = signer,
    to = recipient,
    amount = 10000
  )
```
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/4.png" alt="Alt text" title="Optional title" height="90" width="690">
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/5.png" alt="Alt text" title="Optional title" height="230" width="890">


### 5. initUsers
This instruction is for initialising the Lotery Users. **Note** that, here we are buying tickets, but initialising the User. We are setting a default balance of $20 for each user.
```
@instruction
def init_users(owner:Signer, user:Empty[User], name:str, manager:Manager):
  user=user.init(payer=owner,seeds=['user',owner])
  
  user.name=name
  user.user_add=owner.key()
  user.balance=20 
```

<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/6.png" alt="Alt text" title="Optional title" height="170" width="530">


### 6. buyTickets
This function is used to buy lottery tickets by the initialised Users. We are decrementing ticket price from user's balance and incrementing ticket counter in Manager's Account. Also, we are storing the winner address in Manager's account according to the winner_random number chosen before.
```
@instruction
def buy_tickets(user:User, manager:Manager,signer:Signer, user_token:Empty[TokenAccount],mint:TokenMint):  
  user_token.init(payer = signer, seeds = ['Token', signer],mint = mint, authority = signer)   
  user.balance-=manager.ticket_price
  user.ticket_count+=1                       #increasing both user ticket count and total tickets count
  manager.ticket_count+=1
  if(manager.ticket_count==manager.winner_no):
    manager.winner_address=user.user_add
```

**Note** that in below image, $5 has been deducted from the User's balance(which was 20 initially) and User ticket count is set to 1 for User. Also the ticket count in Manager Account is set to 1 and winner address has been updated correctly (since we chose winner number as 1 for simplicity). 
<div>
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/7.png" alt="Alt text" title="Optional title" height="170" width="500">
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/8.png" alt="Alt text" title="Optional title" height="170" width="500">
</div>

### 7. checkWinner
Here Users can check whether they got selected or not by giving their Public address and Token Accounts. The Tokens are transferred to the winner here. The Manager Account is the authority who will trnasfer the tokens. In this Program 90 % of token tokens is given to the winner and the rest 10 % allocated for the Manager.
```
@instruction
def check_winner(user:User, manager:Manager,user_token:TokenAccount, manager_token:TokenAccount,signer:Signer):
  assert(signer.key()==manager.manager_add),'Manager is authorised to it'        #assert condition so that only manager can call this function
                   
  if(user.user_add==manager.winner_address):
    print("Congrats you have won the lottery!!")
    #We are distributing 90 % tokens to the winner, and the rest 10 % tokens are commission for the manager
    manager_token.transfer( authority = signer, to = user_token, amount = 9000 )
    user.balance+=9000
  else:
    print("Sorry, Try next time.")
```
So Users balance gets updated to $20 (initial) - $5(ticket price) + 9000(lottery prize)=9015.
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/10.png" alt="Alt text" title="Optional title" height="170" width="790">
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/11.png" alt="Alt text" title="Optional title" height="170" width="500">


## Conclusion
Seahorse vastly simplifies developing programs on Solana at the same time it acts as a gateway for python programmers to get familiar with Rust programs as Seahorse automatically generated the Anchor Rust program for you.
