# Hole Protocol

## Process of messaging
Each user have kind of a bin among freenet network,in which anyone can put message. And for each user this bin is his `USK insert` key. Each message signed by sender. And user can answer to sender, buy putting message at sender insert key if he know it.


## user IDs
Each user have unique pseudonym for each user whom he communicates. Thos pseudonyms uses by user to identify who is sender end establish message flow.
The "message flow" is important here.

- [ ] Probably, for each pair of each people we will **not only one** unique  identifier but... I'm not going to do it in further releases. *TODO*

## Message request
To receive a new message we have to send `ClientGet` to our `USK request` key with URI like `/user_id-message_version`. As well, the identifier is consists of  `user_id-message_number`
For example:
```Bash
ClientGet
URI=USK@myMessageBinKey/messages/user5678-3 
Identifier=request-user5678-3
Verbosity=0
ReturnType=direct
EndMessage
```

## Message sending
To send a message to a friend we putting it at his `USK insert` key, with URI like`/my_id-$(number_of_messages_to_this_user + 1)`. And identifier like `user_id-$(number_of_messages_to_this_user + 1)`
For example:

```Bash
ClientGet
URI=USK@myFriendBinKey/messages/user3246-5
Identifier=user3501-5
Verbosity=0
ReturnType=direct
EndMessage
```


## Adding friend
Hole generates file, which alice have to send to bob somehow, and bob have to do the same. This file will contain unique ID and two keys

