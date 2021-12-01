import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { DateTime } from 'luxon';
import { catchError, map, Observable } from 'rxjs';

@Injectable({
    providedIn: 'root'
})
export class HomeService {
    public notes$: Observable<Note[]>;
    constructor(private http: HttpClient) { }

    public getNotes(): Observable<Note[]> {
        this.notes$ = this.http.get<Note[]>('http://localhost:3001/api', { observe: 'response' }).pipe(
            map((response) => response.body ?? []),
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