import React, { useState } from "react";
import Layout, { apiLocation } from "../Layout";
import Topbar from "../components/Topbar";
import MonacoEditor from 'react-monaco-editor';
import fetchEditorThemes from '../EditorThemes';
import toast from 'react-hot-toast';
import {useNavigate} from "react-router-dom"

import { AES } from 'crypto-js';

export default function HomePage() {
    const navigate = useNavigate(); 
    
    let themeList = fetchEditorThemes();

    let [ editorLanguage, setEditorLanguage ] = useState("rust");
    let [ editorTheme, setEditorTheme ] = useState("vs-dark");
    let [ pasteContents, setPasteContents ] = useState("");

    function beforeEditorMount(editor, monaco) {
        themeList.forEach(value => {
            monaco.editor.defineTheme(value.id, value.theme);
        })
    }

    function saveUnencryptedPaste() {
        if (pasteContents === "" || !pasteContents) {
            toast.error('You can\' save an empty paste!');

            return;
        }

        let paste = {
            language: editorLanguage,
            encrypted: false,
            data: pasteContents
        }

        savePaste(paste);
    }

    function saveEncryptedPaste(secret) {
        if (pasteContents === "" || !pasteContents) {
            toast.error('You can\' save an empty paste!');

            return;
        }

        let paste = {
            language: editorLanguage,
            encrypted: true,
            data: AES.encrypt(pasteContents, secret).toString()
        }

        savePaste(paste);
    }

    function savePaste(paste) {
        if (paste.data === "" || !paste.data) {
            toast.error('You can\' save an empty paste!');

            return;
        }

        let promise = fetch(`${apiLocation()}/api/paste`, {
            method: "POST",
            body: JSON.stringify(paste)
        }).then((response) => {
            if (!response.ok) {
                throw response;
            }

            return response.text();
        }).then((pasteId) => {
            navigate(`/${pasteId}`);

            return pasteId;
        });

        toast.promise(promise, {
            loading: "Saving...",
            success: (pasteId) => "Saved paste " + pasteId,
            error: (err) => `Unable to save: ${err.toString()}`
        })
    }

    return (
        <Layout>
            <Topbar 
                onThemeChange={(value) => setEditorTheme(value)}
                onLanguageChange={(value) => setEditorLanguage(value)}
                defaultThemeValue="vs-dark"
                defaultLanguage="rust"
                onCreate={saveUnencryptedPaste}
                onEncrypt={saveEncryptedPaste}
            />
            <MonacoEditor
                language={editorLanguage}
                theme={editorTheme}
                height="95vh"
                onChange={(newValue, event) => setPasteContents(newValue)}
                options={{
                    quickSuggestions: false,
                    selectOnLineNumbers: true
                }}
                editorDidMount={beforeEditorMount}
            />
        </Layout>
    );
}
