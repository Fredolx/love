import { Injectable } from '@angular/core';
import { Interface } from './models/interface';
import { LanClient } from './models/lanClient';

@Injectable({
  providedIn: 'root'
})
export class MemoryService {

  constructor() { }
  selectedInterface?: Interface
  deviceToKill?: LanClient
  delay: number = 1500;
}
