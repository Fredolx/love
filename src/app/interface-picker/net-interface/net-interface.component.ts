import { Component, Input } from '@angular/core';
import { Interface } from '../../models/interface';

@Component({
  selector: 'app-net-interface',
  templateUrl: './net-interface.component.html',
  styleUrl: './net-interface.component.scss'
})
export class NetInterfaceComponent {
    @Input("interface") interface!: Interface
}
