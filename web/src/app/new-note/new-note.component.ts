import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { MatDialogRef } from '@angular/material/dialog';
import { NoteService } from '../note/note.service';

@Component({
  selector: 'app-new-note',
  templateUrl: './new-note.component.html',
  styleUrls: ['./new-note.component.scss']
})
export class NewNoteComponent implements OnInit {
    public form: FormGroup;

    constructor(private ref: MatDialogRef<NewNoteComponent>) { }

    ngOnInit(): void {
        this.form = new FormGroup({
            creator: new FormControl('', [Validators.required]),
            note: new FormControl('', [Validators.required])
        })
    }

    public onSubmit(): void {
        this.ref.close(this.form.value);
    }
}
