import { Component, EventEmitter, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.css'
})
export class SidebarComponent {
  @Output() contactSelected = new EventEmitter<number>();

  searchQuery = '';
  collapsed = false;

  contacts = [
    { id: 1, name: 'Alice', lastMessage: 'See you later 👋', avatar: 'assets/avatars/alice.png' },
    { id: 2, name: 'Bob', lastMessage: 'Got it!', avatar: 'assets/avatars/bob.png' },
    { id: 3, name: 'Charlie', lastMessage: 'How are you?', avatar: 'assets/avatars/charlie.png' },
  ];

  filteredContacts() {
    if (!this.searchQuery.trim()) return this.contacts;
    return this.contacts.filter(c =>
      c.name.toLowerCase().includes(this.searchQuery.toLowerCase())
    );
  }

  selectContact(id: number) {
    this.contactSelected.emit(id);
  }

  toggleSidebar() {
    this.collapsed = !this.collapsed;
  }
}
