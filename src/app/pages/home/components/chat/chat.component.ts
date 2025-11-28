import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-chat',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './chat.component.html',
  styleUrl: './chat.component.css'
})
export class ChatComponent {
  @Input() contactId: number | null = null;

  newMessage = '';
  messages: { [key: number]: { sender: string; text: string; type: 'sent' | 'received' }[] } = {
    1: [
      { sender: 'Alice', text: 'Hello!', type: 'received' },
      { sender: 'Me', text: 'Hi Alice!', type: 'sent' },
    ],
    2: [
      { sender: 'Bob', text: 'Did you get it?', type: 'received' },
      { sender: 'Me', text: 'Yes, thanks!', type: 'sent' },
    ],
    3: [
      { sender: 'Charlie', text: 'How are you?', type: 'received' },
      { sender: 'Me', text: 'I’m good, you?', type: 'sent' },
    ],
  };

  sendMessage() {
    if (!this.newMessage.trim() || this.contactId === null) return;
    this.messages[this.contactId].push({ sender: 'Me', text: this.newMessage, type: 'sent' });
    this.newMessage = '';
  }
}
