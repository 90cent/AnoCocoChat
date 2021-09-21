# CocoChat
EndToEnd Anonymus chatting app. With Rust Backend and Android support. +Webapp



Roadmap:

1 - Backend. (Routed trought tor)  
2. Webapp  
3. Windows Application  
4. Android App  
5. IOS Optimized Webapp ( :( apple doesn't like windows users, thats why an webapp :( )  


How it should work:

In this "App" nobody is requiered to use a Username or Email.  
Its like a Crypto Wallet, a random 26 word key gets generated and hashed and then sent to a database, the hash (that name key) will be saved in a database with a random SHA256.  

The SHA256 will be called (UserID). 

But who wants an 256 character long ID to identify people.

Thats where the "AnoNumber" comes in place, this is a 7 digit number like a username but not tied to one. And every 6 Hours the numbers will be increased by one.


Here is an Example:

Your AnoNumber 6:00 (AM) = 500000
Your AnoNumber 12:00 (AM) = 500001


But what if my number reaches 9999999 ?
  it will begin at 1 if not avalible you will have to wait 6 Hours
 



But why is the number changing ?
  To make you more untracable.
  
 
 
 
 
 
 
 Planed features for CocoChat:
  - Encrypted Texting (surprise)
  - The Message will stay encrypted until the reciever opens the app and presses on the message to enter thier own password
  - Everything is routed trough tor. 
  - Onion Webapp and Normal Webapp (not that secure for you :( )
  - Voice Messaging (and options to pitch your voice)
  - Bluetooth Messaging if you dont have internet but your friend does your messages get routed trough his phone. (He will not see them)
