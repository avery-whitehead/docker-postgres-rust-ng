import { Component, Input, OnInit } from '@angular/core';
import { Note } from '../home/home.service';

@Component({
  selector: 'app-note',
  templateUrl: './note.component.html',
  styleUrls: ['./note.component.scss']
})
export class NoteComponent implements OnInit {
  @Input() public note: Note;

  constructor() { }

  ngOnInit(): void { }

}
