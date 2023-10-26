import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';
import Navbar from 'react-bootstrap/Navbar';
import InputGroup from 'react-bootstrap/InputGroup';
import fetchEditorThemes from '../EditorThemes';

const DEFAULT_EDITOR_THEMES = ["vs-dark", "vs-light", "hc-black", "hc-light"];

export default function Topbar(props) {
    let themeList = fetchEditorThemes();

    let selectOptions = themeList.map(value => (<option value={value.id}>{value.name}</option>));
    selectOptions.push((<option value="vs-dark">Visual Studio Dark</option>));
    selectOptions.push((<option value="vs-light">Visual Studio Light</option>));
    selectOptions.push((<option value="hc-black">High Contrast Black</option>));
    selectOptions.push((<option value="hc-light">High Contrast Light</option>));

    return (
        <div className="border border-primary">
            <Navbar className="d-flex justify-content-between">
                    <Navbar.Brand href="/">
                        <img
                            src={process.env.PUBLIC_URL + '/jar.svg'}
                            width="30"
                            height="30"
                            className="d-inline-block align-top"
                            alt="React Bootstrap logo"
                        />
                    </Navbar.Brand>
                    <Form>
                        <InputGroup>
                            <InputGroup.Text id="editor-language">Language</InputGroup.Text>
                            <Form.Select 
                                disabled={props.paste != null} 
                                value={props.paste?.language?.toLowerCase()} 
                                aria-label="Language"
                                onChange={(event) => props.onLanguageChange(event.target.value)}
                            >
                                <option value="plaintext">None</option>
                                <option value="csharp">C#</option>
                                <option value="java">Java</option>
                                <option value="go">Go</option>
                                <option value="rust">Rust</option>
                                <option value="cpp">C++</option>
                                <option value="python">Python</option>
                            </Form.Select>
                        </InputGroup>
                    </Form>
                    <Form>
                        <InputGroup>
                            <InputGroup.Text id="editor-theme">Theme</InputGroup.Text>
                            <Form.Select 
                                aria-label="Theme"
                                type="select"
                                onChange={(event) => props.onThemeChange(event.target.value)}
                                defaultValue={props.defaultThemeValue}
                            >
                                {
                                    selectOptions
                                }
                            </Form.Select>
                        </InputGroup>
                    </Form>
                    {
                        props.paste?.encrypted && 
                        <Form>
                            <InputGroup>
                                <Button variant="outline-secondary" id="button-decrypt">
                                    Decrypt
                                </Button>
                                <Form.Control
                                    aria-label="Decrypt the paste"
                                />
                            </InputGroup>
                        </Form>
                    }
                    {
                        !props.paste && 
                        <Form>
                            <InputGroup>
                                <Button variant="outline-secondary" id="button-encrypt">
                                    Encrypt
                                </Button>
                                <Form.Control
                                    aria-label="Encrypt the paste and create it"
                                />
                            </InputGroup>
                        </Form>
                    }
                    {
                        props.paste != null && 
                        <Form>
                            <Button variant="outline-danger" id="button-delete">
                                Delete
                            </Button>
                        </Form>
                    }
                    {
                        !props.paste && 
                        <Form>
                            <Button variant="outline-primary" id="button-create">
                                Create
                            </Button>
                        </Form>
                    }
            </Navbar>
        </div>
    );
}