**This project was created by Seahorse 0.2.2.**

# Seahorse Lottery Program
The Objective of this project is to create a **Lottery smart contract** where users can buy lottery tickets and a random winner will be selected from all participants and lottery prize will be rewarded to the winner.

## Prerequisites
We need to install some command line tools for this project to build. We need [Solana](https://docs.solana.com/cli/install-solana-cli-tools), [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html#install-rust), [NodeJS](https://nodejs.org/en/) and [Seahorse](https://seahorse-lang.org/docs/installation). The links provided contain the step-by-step guide on installing these tools and the dependencies required for them like Rust.

You can check if package got installed correctly by running command like :

solana -V
anchor -V
seahorse -V

I am using:
* solana 1.9.2

* anchor 0.25.0

* seahorse v0.2.2

* node 16.8.0

## Involved Parties
There are 2 types of entities involved here, which are **Manager** and **User**. The Manager is responsible for creating the Lottery and its parameters like lottery price and calling the random winner from all lottery buyers. On the other hand, Users are the people who are buying the lottery. 

Manager Account has fields like Name, Public address, ticket_price, total ticket_count, winner_no(a random no for deciding winner), winner_address(for storing the winner Public key). User Account has fields like Username, user address, ticket_count(number of tickets user bought), User balance.

<div>
  
<img src="https://github.com/akshaydhayal/Seahorse-Lottery/blob/master/assets/ManagerDetail.png" alt="Alt text" title="Optional title" height="180" width="240">
  
<img src="https://github.com/akshaydhayal/Seahorse-Lottery/blob/master/assets/UserDetail.png" alt="Alt text" title="Optional title" height="180" width="240">
<!-- <img src="https://github.com/akshaydhayal/Seahorse-Lottery/blob/master/assets/Instructions.png" alt="Alt text" title="Optional title" height="260" width="180">
 -->
</div>

## Program Instructions
We have 7 functions/instructions in this Program. Let's understand all the different Program instructions one by one.

### 1. ManagerInit
This is the function where we **initialise** the Manager's Account. We set the lottery price here(took $5 here) , ticket counter as 0 and the random number(which will be used for winner selection). Winner address is garbage value at start, will be updated at later functions.
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/1.png" alt="Alt text" title="Optional title" height="120" width="380">

### 2. initTokenMint  
This is where we mint.

<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/2.png" alt="Alt text" title="Optional title" height="120" width="480">

### 3. managerTokenAcc
Initialise a Token Account for Manager Account so that it can hold Mint Tokens. Now, its just showing 0 Tokens as we have not minted our tokens.

<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/3.png" alt="Alt text" title="Optional title" height="90" width="690">

### 4. useTokenMint
In this function, we are minting **10000 Tokens** to the Manager Token Account.

<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/4.png" alt="Alt text" title="Optional title" height="90" width="690">
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/5.png" alt="Alt text" title="Optional title" height="230" width="890">


### 5. initUsers
This instruction is for initialising the Lotery Users. **Note** that, here we are buying tickets, but initialising the User. We are setting a defaul balance of $20 for each user.

<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/6.png" alt="Alt text" title="Optional title" height="170" width="530">


### 6. buyTickets
This function is used to buy lottery tickets by the initialised Users. **Note** that in below image, $5 has been deducted from the balance and User ticket count is set to 1 for User. Also the ticket count in Manager Account is set to 1 and winner address has been updated correctly (since we chose winner number as 1 for simplicity). 

<div>
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/7.png" alt="Alt text" title="Optional title" height="170" width="500">
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/8.png" alt="Alt text" title="Optional title" height="170" width="500">
</div>

### 7. checkWinner
Here Users can check whether they got selected or not by giving their Public address and Token Accounts. The Tokens are transferred to the winner here. In this Program 90 % of token tokens is given to the winner and the rest 10 % allocated for the Manager. So Users balance gets updated to $20 (initial) - $5(ticket price) + 9000(lottery prize)=9015.

<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/10.png" alt="Alt text" title="Optional title" height="170" width="790">
<img src="https://github.com/akshaydhayal/Seahorse-Lottery-Program/blob/master/assets/11.png" alt="Alt text" title="Optional title" height="170" width="500">


