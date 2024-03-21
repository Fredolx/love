import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NpcapComponent } from './npcap.component';

describe('NpcapComponent', () => {
  let component: NpcapComponent;
  let fixture: ComponentFixture<NpcapComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [NpcapComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(NpcapComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
