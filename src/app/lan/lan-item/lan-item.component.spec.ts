import { ComponentFixture, TestBed } from '@angular/core/testing';

import { LanItemComponent } from './lan-item.component';

describe('LanItemComponent', () => {
  let component: LanItemComponent;
  let fixture: ComponentFixture<LanItemComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [LanItemComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(LanItemComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
