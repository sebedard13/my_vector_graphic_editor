import { Injectable } from '@angular/core';
import { Selected } from 'wasm-vgc';

@Injectable({
  providedIn: 'root'
})
export class SelectionService {

  public selection: Selected = new Selected();

  constructor() { }
}
