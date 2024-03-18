import { Component, Input, OnDestroy, OnInit } from '@angular/core';
import { LanClient } from '../../models/lanClient';
import { MemoryService } from '../../memory.service';
import { invoke } from '@tauri-apps/api';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-lan-item',
  templateUrl: './lan-item.component.html',
  styleUrl: './lan-item.component.scss'
})
export class LanItemComponent implements OnInit, OnDestroy {
  killing: boolean = false;
  canKillStop: boolean = false;
  cancelSignal?: Subscription;
  @Input("lanClient") lanClient!: LanClient
  constructor(private memory: MemoryService) {
  }

  ngOnInit(): void {
      this.cancelSignal = this.memory.cancelSubject.subscribe(_ => {
        if(this.killing)
          console.log("signal accepted");
          this.stopKill();
      });
  }

  kill() {
    if (this.killing)
      return;
    this.killing = true;
    this.canKillStop = true;
    invoke("kill_device", {
      client: this.lanClient,
      inter: this.memory.selectedInterface?.name,
      delay: this.memory.delay
    }).then().finally(() => {
      this.killing = false
    });
  }

  stopKill() {
    if (!this.canKillStop)
      return;
    this.canKillStop = false;
    invoke("stop_kill_device", {
      client: this.lanClient
    }).then().catch((e) => console.error("failed to stop kill, " + e))
  }

  ngOnDestroy(): void {
    this.cancelSignal?.unsubscribe();
  }
}
