import { Component, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { Interface } from '../models/interface';
import { MemoryService } from '../memory.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-interface-picker',
  templateUrl: './interface-picker.component.html',
  styleUrl: './interface-picker.component.scss'
})
export class InterfacePickerComponent implements OnInit {
  interfaces: Interface[] = []

  constructor(private memory: MemoryService, private router: Router) {
    
  }

  ngOnInit(): void {
    invoke('get_interfaces')
      .then((response) => this.interfaces = response as Interface[])
  }

  selectInterface(inter: Interface) {
    this.memory.selectedInterface = inter;
    this.router.navigateByUrl("lan");
  }
}
