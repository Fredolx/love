import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { NetInterfaceComponent } from './net-interface/net-interface.component';
import { InterfacePickerComponent } from './interface-picker/interface-picker.component';

@NgModule({
  declarations: [
    AppComponent,
    NetInterfaceComponent,
    InterfacePickerComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
