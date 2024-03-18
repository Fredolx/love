import { Injectable } from '@angular/core';
import { Interface } from './models/interface';
import { LanClient } from './models/lanClient';
import { Subject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class MemoryService {

  constructor() { }
  selectedInterface?: Interface
  deviceToKill?: LanClient
  delay: number = 1500;
  cancelSubject: Subject<boolean> = new Subject<boolean>(); 
  skippedSelectInteface: boolean = false;
}
