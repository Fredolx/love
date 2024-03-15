import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { InterfacePickerComponent } from './interface-picker/interface-picker.component';
import { LanComponent } from './lan/lan.component';

const routes: Routes = [
  {path: "", component: InterfacePickerComponent},
  {path: "lan", component: LanComponent}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
