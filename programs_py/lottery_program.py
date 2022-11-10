# lottery_program
# Built with Seahorse v0.2.2

from seahorse.prelude import *

# This is your program's public key and it will update
# automatically when you build the project.

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')



#Manager Account who is reponsible for creating the lottery parameter(like price,total tickets) and also deciding the winner.Although winner is decided 
#randomly but for simplicity we have taken winner_no as a random parameter for winner selection.

class Manager(Account):
  name:str
  manager_add:Pubkey
  ticket_price:u64
  ticket_count:u64
  winner_no:u64
  winner_address:Pubkey

  
#This is Users Account which have ticket count and users balance parameters.
class User(Account):
  name:str
  user_add:Pubkey
  ticket_count:u64
  balance:u64

#Initialising the Manager Account, here we are passing the random no for winner selection.
@instruction
def manager_init(owner:Signer, name:str, manager:Empty[Manager], winner_random_no:u64):
  manager=manager.init(payer=owner,seeds=['manager',owner])

  manager.manager_add=owner.key()
  manager.name=name
  manager.ticket_price=5                #Assuming ticket price as 5$.
  manager.ticket_count=0
  manager.winner_no=winner_random_no

@instruction
def manager_token_acc(manager_token_acc:Empty[TokenAccount],mint: TokenMint,signer: Signer):
  manager_token_acc.init(payer = signer, seeds = ['token', signer],
                         mint = mint, authority = signer)

  
#Initialsing the Users Account, passing as User's name and Manager Account. 
@instruction
def init_users(owner:Signer, user:Empty[User], name:str, manager:Manager):
  user=user.init(payer=owner,seeds=['user',owner])
  
  user.name=name
  user.user_add=owner.key()
  user.balance=20                              #For simplicity, initialising users starting balance as 20$.


#This is function for buying lottery tickets from initialised users accounts.
@instruction
def buy_tickets(user:User, manager:Manager,signer:Signer,
               user_token:Empty[TokenAccount],mint:TokenMint):  
  
  user_token.init(payer = signer, seeds = ['Token', signer],mint = mint, authority = signer)   

  user.balance-=manager.ticket_price
  user.ticket_count+=1                       #increasing both user ticket count and total tickets count
  manager.ticket_count+=1
  if(manager.ticket_count==manager.winner_no):
    manager.winner_address=user.user_add
    

#Function where Users can check whether they have won or not using their public addresses.
@instruction
def check_winner(user:User, manager:Manager,user_token:TokenAccount,
                 manager_token:TokenAccount,signer:Signer):
  
  assert(signer.key()==manager.manager_add),'Manager is authorised to it'        #assert condition so that only manager can call this function
                   
  if(user.user_add==manager.winner_address):
    print("Congrats you have won the lottery!!")
    #We are distributing 90 % tokens to the winner, and the rest 10 % tokens are commission for the manager
    manager_token.transfer( authority = signer, to = user_token, amount = 9000 )
    user.balance+=9000
  else:
    print("Sorry, Try next time.")


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
  

@instruction
def use_token_mint(mint: TokenMint,recipient: TokenAccount,signer: Signer,manager:Manager):
  # Minting 10000 tokens from our `mint` to `recipient` i.e Manager.
  # Note that the amounts here are in *native* token quantities - you need to

  assert(signer.key()==manager.manager_add),"Only Manager is authorised to this function"  #assert 3
  mint.mint(
    authority = signer,
    to = recipient,
    amount = 10000
  )

