import { Injectable } from '@angular/core';
import { Interface } from './models/interface';

@Injectable({
  providedIn: 'root'
})
export class MemoryService {

  constructor() { }
  selectedInterface?: Interface
}
