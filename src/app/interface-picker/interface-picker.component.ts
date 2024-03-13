import { Component, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { Interface } from '../models/interface';

@Component({
  selector: 'app-interface-picker',
  templateUrl: './interface-picker.component.html',
  styleUrl: './interface-picker.component.scss'
})
export class InterfacePickerComponent implements OnInit {
  interfaces: Interface[] = []
  ngOnInit(): void {
    invoke('get_interfaces')
    .then((response) => this.interfaces = response as Interface[])
  }
}
