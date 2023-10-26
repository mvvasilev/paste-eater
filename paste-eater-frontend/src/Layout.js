import ThemeProvider from 'react-bootstrap/ThemeProvider';
import { Toaster } from 'react-hot-toast';

function Layout({children}) {
    return (
        <main>
            <ThemeProvider breakpoints={['xxxl', 'xxl', 'xl', 'lg', 'md', 'sm', 'xs', 'xxs']} minBreakpoint='lg'>
                <div className="container-lg themed-container">
                    {children}
                </div>
                <Toaster 
                    toastOptions={{
                        success: {
                            style: {
                            background: '#dad7cd',
                            },
                        },
                        error: {
                            style: {
                            background: '#ff8fab',
                            },
                        },
                    }}
                />
            </ThemeProvider>
        </main>
    );
}

export default Layout;