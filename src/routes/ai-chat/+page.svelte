<script lang="ts">
  import { onMount } from "svelte";
  
  // Mock chat history
  let chatHistory = [
    { 
      sender: 'ai', 
      message: 'こんにちは！AI家計簿アシスタントです。家計簿についてなにかお手伝いできることはありますか？' 
    }
  ];
  
  let newMessage = "";
  let chatContainer;
  
  function sendMessage() {
    if (!newMessage.trim()) return;
    
    // Add user message to chat
    chatHistory = [...chatHistory, { sender: 'user', message: newMessage }];
    const userQuestion = newMessage;
    newMessage = "";
    
    // Simulate AI response (in a real app, this would call an API)
    setTimeout(() => {
      // Mock AI responses based on keywords in the user's message
      let aiResponse = "";
      
      if (userQuestion.includes('支出') || userQuestion.includes('使いすぎ')) {
        aiResponse = "今月の支出は計127,500円で、食費が最も大きい支出カテゴリとなっています。前月比で15%減少しているので、支出管理は順調です。";
      } else if (userQuestion.includes('収入') || userQuestion.includes('給料')) {
        aiResponse = "今月の収入は合計350,000円です。主な収入源は給与の280,000円と副業収入の70,000円です。";
      } else if (userQuestion.includes('貯金') || userQuestion.includes('貯蓄')) {
        aiResponse = "現在の貯蓄率は収入の約20%です。目標の30%には達していませんが、3ヶ月前の15%から改善しています。";
      } else if (userQuestion.includes('予算') || userQuestion.includes('予算管理')) {
        aiResponse = "今月は食費の予算50,000円に対して実績が42,500円で、予算内に収まっています。一方、娯楽費は予算15,000円に対して20,000円と超過しています。";
      } else {
        aiResponse = "ご質問の詳細をもう少し教えていただけますか？例えば、特定の期間や支出カテゴリについて知りたい場合は、「先月の食費はいくらだった？」のように質問していただくとお答えしやすいです。";
      }
      
      chatHistory = [...chatHistory, { sender: 'ai', message: aiResponse }];
      
      // Scroll to bottom of chat after new message
      setTimeout(() => {
        if (chatContainer) {
          chatContainer.scrollTop = chatContainer.scrollHeight;
        }
      }, 0);
    }, 1000);
  }
  
  onMount(() => {
    // Set focus to input when component mounts
    document.querySelector('textarea')?.focus();
  });
  
  // Handle enter key to send message
  function handleKeydown(event) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      sendMessage();
    }
  }
</script>

<div class="ai-chat-container">
  <div class="chat-header">
    <h1>AIチャット</h1>
    <p>家計簿の分析や管理について、AIにチャットで質問できます。</p>
  </div>
  
  <div class="chat-window">
    <div class="chat-messages" bind:this={chatContainer}>
      {#each chatHistory as chat}
        <div class="message {chat.sender}">
          <div class="message-bubble">
            {chat.message}
          </div>
          <div class="message-avatar">
            {#if chat.sender === 'ai'}
              <span class="ai-avatar">AI</span>
            {:else}
              <span class="user-avatar">👤</span>
            {/if}
          </div>
        </div>
      {/each}
    </div>
    
    <div class="chat-input">
      <textarea 
        bind:value={newMessage} 
        on:keydown={handleKeydown}
        placeholder="AIに質問してください..."
      ></textarea>
      <button on:click={sendMessage} disabled={!newMessage.trim()}>
        送信
      </button>
    </div>
  </div>
  
  <div class="chat-tips">
    <h3>質問例:</h3>
    <ul>
      <li>「今月の支出はいくら？」</li>
      <li>「食費が予算をオーバーしている？」</li>
      <li>「貯金率を上げるにはどうしたらいい？」</li>
      <li>「先月と比べて支出はどう変化した？」</li>
    </ul>
  </div>
</div>

<style>
  .ai-chat-container {
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  
  .chat-header {
    margin-bottom: 1rem;
  }
  
  .chat-header h1 {
    margin-bottom: 0.5rem;
  }
  
  .chat-window {
    display: flex;
    flex-direction: column;
    flex: 1;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    background-color: white;
    overflow: hidden;
    min-height: 500px;
  }
  
  .chat-messages {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .message {
    display: flex;
    align-items: flex-start;
    margin-bottom: 0.5rem;
  }
  
  .message.user {
    flex-direction: row-reverse;
  }
  
  .message-bubble {
    max-width: 70%;
    padding: 0.75rem 1rem;
    border-radius: 1rem;
    position: relative;
    word-break: break-word;
  }
  
  .message.ai .message-bubble {
    background-color: #f0f4f8;
    color: var(--light-text);
    border-top-left-radius: 0.25rem;
    margin-right: 0.5rem;
  }
  
  .message.user .message-bubble {
    background-color: var(--primary);
    color: white;
    border-top-right-radius: 0.25rem;
    margin-left: 0.5rem;
  }
  
  .message-avatar {
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    flex-shrink: 0;
  }
  
  .ai-avatar {
    background-color: var(--accent);
    color: white;
    width: 2rem;
    height: 2rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
  }
  
  .user-avatar {
    font-size: 1.5rem;
  }
  
  .chat-input {
    display: flex;
    border-top: 1px solid var(--light-border);
    padding: 1rem;
    background-color: white;
  }
  
  .chat-input textarea {
    flex: 1;
    padding: 0.75rem;
    border: 1px solid var(--light-border);
    border-radius: var(--radius-md);
    resize: none;
    height: 80px;
    margin-right: 0.5rem;
  }
  
  .chat-input button {
    align-self: flex-end;
    height: 40px;
  }
  
  .chat-tips {
    background-color: white;
    border-radius: var(--radius-lg);
    padding: 1rem;
    margin-top: 1rem;
    box-shadow: var(--shadow-md);
  }
  
  .chat-tips h3 {
    font-size: 1rem;
    margin-bottom: 0.5rem;
  }
  
  .chat-tips ul {
    list-style: disc;
    padding-left: 1.5rem;
  }
  
  .chat-tips li {
    margin-bottom: 0.25rem;
  }
</style>
