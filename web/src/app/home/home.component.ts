import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { Observable } from 'rxjs';
import { NewNoteComponent } from '../new-note/new-note.component';
import { HomeService, Note } from './home.service';


@Component({
    selector: 'app-home',
    templateUrl: './home.component.html',
    styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {
    public notes$: Observable<Note[]>;

    constructor(private homeService: HomeService, private dialogue: MatDialog) {}

    ngOnInit() {
        this.notes$ = this.homeService.getNotes();
    }

    public openNoteDialogue() {
        this.dialogue.open(NewNoteComponent);
    }
}
