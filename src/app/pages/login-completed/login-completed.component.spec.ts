import { ComponentFixture, TestBed } from '@angular/core/testing';

import { LoginCompletedComponent } from './login-completed.component';

describe('LoginCompletedComponent', () => {
  let component: LoginCompletedComponent;
  let fixture: ComponentFixture<LoginCompletedComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LoginCompletedComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(LoginCompletedComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
