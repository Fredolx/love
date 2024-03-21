import { Component } from '@angular/core';
import { open } from '@tauri-apps/api/shell';
@Component({
  selector: 'app-npcap',
  templateUrl: './npcap.component.html',
  styleUrl: './npcap.component.scss'
})
export class NpcapComponent {
  openLink() {
    open('https://npcap.com/#download').then();
  }
}
