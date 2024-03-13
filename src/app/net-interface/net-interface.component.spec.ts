import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NetInterfaceComponent } from './net-interface.component';

describe('NetInterfaceComponent', () => {
  let component: NetInterfaceComponent;
  let fixture: ComponentFixture<NetInterfaceComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [NetInterfaceComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(NetInterfaceComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
