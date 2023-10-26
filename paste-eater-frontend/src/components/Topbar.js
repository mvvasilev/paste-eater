import React, { useState } from "react";
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';
import Navbar from 'react-bootstrap/Navbar';
import InputGroup from 'react-bootstrap/InputGroup';
import fetchEditorThemes from '../EditorThemes';
import metadata from 'monaco-editor/esm/metadata';
import Image from 'react-bootstrap/Image';

export default function Topbar(props) {
    let themeList = fetchEditorThemes();

    let [ encryptionKey, setEncryptionKey ] = useState("");
    let [ decryptionKey, setDecryptionKey ] = useState("");

    let selectOptions = themeList.map(value => (<option value={value.id}>{value.name}</option>));
    selectOptions.push((<option value="vs-dark">Visual Studio Dark</option>));
    selectOptions.push((<option value="vs-light">Visual Studio Light</option>));
    selectOptions.push((<option value="hc-black">High Contrast Black</option>));
    selectOptions.push((<option value="hc-light">High Contrast Light</option>));

    return (
        <div className="bg-primary">
            <Navbar className="d-flex justify-content-between">
                    <Navbar.Brand className="ms-2" href="/">
                        <Image
                            src={process.env.PUBLIC_URL + '/jar.svg'}
                            width="50"
                            height="50"
                            className="d-inline-block align-top bg-dark"
                            alt="Jar"
                            roundedCircle
                        />
                    </Navbar.Brand>
                    <Form className="me-2">
                        <InputGroup>
                            <InputGroup.Text id="editor-language">Language</InputGroup.Text>
                            <Form.Select 
                                disabled={props.paste != null} 
                                defaultValue={props.paste?.language?.toLowerCase() ?? props.defaultLanguage ?? "rust"} 
                                aria-label="Language"
                                onChange={(event) => props.onLanguageChange(event.target.value)}
                            >
                                {
                                    metadata.languages.map(language => (<option value={language.label}>{language.label}</option>))
                                }
                            </Form.Select>
                        </InputGroup>
                    </Form>
                    <Form className="me-2">
                        <InputGroup>
                            <InputGroup.Text id="editor-theme">Theme</InputGroup.Text>
                            <Form.Select 
                                aria-label="Theme"
                                type="select"
                                onChange={(event) => props.onThemeChange(event.target.value)}
                                defaultValue={props.defaultThemeValue ?? "vs-dark"}
                            >
                                {
                                    selectOptions
                                }
                            </Form.Select>
                        </InputGroup>
                    </Form>
                    {
                        props.paste?.encrypted && 
                        <Form className="me-2">
                            <InputGroup>
                                <Button 
                                    variant="secondary" 
                                    id="button-decrypt"
                                    onClick={() => props.onDecrypt(decryptionKey)}
                                >
                                    Decrypt
                                </Button>
                                <Form.Control
                                    id="decryption-key"
                                    onChange={(event) => setDecryptionKey(event.target.value)}
                                    aria-label="Decrypt the paste"
                                />
                            </InputGroup>
                        </Form>
                    }
                    {
                        !props.paste && 
                        <Form className="me-2">
                            <InputGroup>
                                <Button 
                                    variant="secondary" 
                                    id="button-encrypt"
                                    onClick={() => props.onEncrypt(encryptionKey)}
                                >
                                    Encrypt
                                </Button>
                                <Form.Control
                                    id="encryption-key"
                                    onChange={(event) => setEncryptionKey(event.target.value)}
                                    aria-label="Encrypt the paste and create it"
                                />
                            </InputGroup>
                        </Form>
                    }
                    {
                        !props.paste && 
                        <Form className="me-2">
                            <Button 
                                variant="success" 
                                className="me-2"
                                id="button-create"
                                onClick={props.onCreate}
                                disabled={encryptionKey}
                            >
                                Create
                            </Button>
                        </Form>
                    }
            </Navbar>
        </div>
    );
}