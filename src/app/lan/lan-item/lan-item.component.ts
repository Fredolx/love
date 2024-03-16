import { Component, Input } from '@angular/core';
import { LanClient } from '../../models/lanClient';
import { MemoryService } from '../../memory.service';

@Component({
  selector: 'app-lan-item',
  templateUrl: './lan-item.component.html',
  styleUrl: './lan-item.component.scss'
})
export class LanItemComponent {
  @Input("lanClient") lanClient!: LanClient
  constructor(private memory: MemoryService) {
  }
  beingKilled() {
    return this.memory.deviceToKill == this.lanClient
  }
}
