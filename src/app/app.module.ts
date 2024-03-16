import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { InterfacePickerComponent } from './interface-picker/interface-picker.component';
import { NetInterfaceComponent } from './interface-picker/net-interface/net-interface.component';
import { LanComponent } from './lan/lan.component';
import { LanItemComponent } from './lan/lan-item/lan-item.component';
import { LoadingComponent } from './loading/loading.component';

@NgModule({
  declarations: [
    AppComponent,
    NetInterfaceComponent,
    InterfacePickerComponent,
    LanComponent,
    LanItemComponent,
    LoadingComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
