import { ComponentFixture, TestBed } from '@angular/core/testing';

import { InterfacePickerComponent } from './interface-picker.component';

describe('InterfacePickerComponent', () => {
  let component: InterfacePickerComponent;
  let fixture: ComponentFixture<InterfacePickerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [InterfacePickerComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(InterfacePickerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
