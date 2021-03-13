# Message flow
*(pseudocode in Bash style)*

Hole receiving messages by looking for new items in "user's bin" (place where new messages arrives). Because of how freenet works, we can't just say "Hey! Do I have new messages?", we can specify messages index like: "Hey! Gimme 15th message". Thus we must remember previous message index and to receive the new message we're requesting for 16th message, if we was able to get 15th message.

That's mean, what interlocutor also have to know the last message index in our bin. But **how he can know it, if there's  a lot of other users who also sending messages to our bin?** Of course, you can try to receive all messages from the index, you had  last remembered, and next index, next, next... until you find the index which does not exist. But it's a freenet, it's take too much time;

The solution is unique Id. User creating and giving his/her unique id to each new interlocutor. And when, if interlocutor sending the message he knows, that last message to us had index `$N` and sends message with index `$N+1`, if be more verbose, the new message index he send to us will look like `"$(ID)_($N+1)"` 
