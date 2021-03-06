import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { Observable, switchMap } from 'rxjs';
import { NewNoteComponent } from '../new-note/new-note.component';
import { NoteService, Note } from '../note/note.service';


@Component({
    selector: 'app-home',
    templateUrl: './home.component.html',
    styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {
    public notes$: Observable<Note[]>;

    constructor(private noteService: NoteService, private dialogue: MatDialog) {}

    ngOnInit() {
        this.notes$ = this.noteService.getNotes();
    }

    public openNoteDialogue() {
        const ref = this.dialogue.open(NewNoteComponent);
        this.notes$ = ref.afterClosed().pipe(
            switchMap((event: Note) => this.noteService.addNote(event))
        );
    }
}
