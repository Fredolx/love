import { Component, OnInit } from '@angular/core';
import { LanClient } from '../models/lanClient';
import { cli, invoke } from '@tauri-apps/api';
import { MemoryService } from '../memory.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-lan',
  templateUrl: './lan.component.html',
  styleUrl: './lan.component.scss'
})
export class LanComponent implements OnInit {
  lanClients: LanClient[] = []
  loading: boolean = true;

  constructor(public memory: MemoryService, private router: Router) {

  }

  ngOnInit(): void {
    if (!this.memory.selectedInterface)
      this.router.navigateByUrl("");
    this.getLan();
  }

  goBack() {
    this.memory.cancelSubject.next(true);
    this.memory.selectedInterface = undefined;
    this.router.navigateByUrl("");
  }

  getLan() {
    this.loading = true;
    invoke("get_lan", {
      inter: this.memory.selectedInterface?.name
    })
      .then((x: any) => this.lanClients = x)
      .finally(() => {
        this.loading = false;
      });
  }
}
