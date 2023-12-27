import { Injectable } from '@angular/core';
import { Subject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class MouseInfoService {


  public coords = new Subject<{x: number, y: number}>();
  public zoom = new Subject<number>();

  constructor() { }
}