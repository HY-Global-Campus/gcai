<script lang="ts">
  import axios from 'axios';
  import { onMount } from 'svelte';

  interface Message {
    role: 'system' | 'user' | 'assistant';
    content: string;
  }

  interface Choice {
    messages: Message[];
    id: string;
    created: number;
  }

  let userInput: string = "";
  let courseName: string = "";
  let assistantName: string = "";
  let welcomeMessage: string = "Hello! How can I help today?";
  let instructions: string = "This is a default, if you receive this, just reponds with 'instructions not received'";

  let messages: Message[] = [
    { role: "system", content: instructions },
  ];

  async function sendMessage(): Promise<void> {
    messages[0].content = instructions;
   messages = [...messages, { role: "user", content: userInput }];
    const payload = {
      messages: messages 
    };
    console.log(payload);

    userInput = ""; // Clear the input field
    try {
      // Send the request to the proxy endpoint instead of the external API
      const response = await axios.post('/api', payload);

      
      // Assuming the server response structure matches what the client expects
      console.log(response.data);
      response.data.choices.map((choice: Choice) => {
        choice.messages.map((message: Message) => {
            messages = [...messages, { role: "assistant", content: message.content }];
          
        })
      }) 
      
    } catch (error) {
      console.error("Failed to send message through proxy:", error);
    }
  }

  onMount(() => {
    // Initialization logic if needed
  });
</script>


<style>
  .chat-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
  }

  .message {
    padding: 10px;
    border-radius: 5px;
    margin: 5px;
  }

  .system {
    background-color: #f0f0f0;
  }

  .user {
    background-color: #e0e0e0;
  }

  .assistant {
    background-color: #d0d0d0;
  }
</style>
  Course name
  <input type="text"
    bind:value={courseName}
    placeholder="Type a message..."
  >

  Assistant name
  <input type="text"
    bind:value={assistantName}
    placeholder="Type a message..."
  >

  Welcome message
  <input type="text"
    bind:value={welcomeMessage}
    placeholder="Type a message..."
  >

  Instructions
  <input type="text"
    bind:value={instructions}
    placeholder="Type a message..."
  >
<div class="chat-container">
  {#each messages as message }
    {#if (message.role === "user" || message.role === "assistant")}
      <div class="message system">
        {message.content}
      </div>
    {/if}
  {/each}


  Test your assistant
  <input
    type="text"
    bind:value={userInput}
    on:keydown={(e) => e.key === 'Enter' && sendMessage()}
    placeholder="Type a message..."
  >

  <button on:click={sendMessage}>Send</button>
</div>
