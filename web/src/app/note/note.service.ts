import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { DateTime } from 'luxon';
import { catchError, map, Observable } from 'rxjs';

@Injectable({
    providedIn: 'root'
})
export class NoteService {
    public notes$: Observable<Note[]>;
    constructor(private http: HttpClient) { }

    public getNotes(): Observable<Note[]> {
        this.notes$ = this.http.get<any>('http://localhost:3001/api/notes', { observe: 'response' }).pipe(
            map((response) => {
                response.body?.map((note: any) => {
                    note.ts = DateTime.fromISO(note.ts);
                })
                return response.body ?? [];
            }),
            catchError((err) => {
                console.warn(err);
                return [];
            })
        )
        return this.notes$;
    }
}

export interface Note {
    id: number;
    creator: string;
    note: string;
    ts: DateTime;
}