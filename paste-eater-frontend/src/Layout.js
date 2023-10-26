import ThemeProvider from 'react-bootstrap/ThemeProvider';

function Layout({children}) {
    return (
        <main>
            <ThemeProvider breakpoints={['xxxl', 'xxl', 'xl', 'lg', 'md', 'sm', 'xs', 'xxs']} minBreakpoint='lg'>
                <div className="container-lg themed-container">
                    {children}
                </div>
            </ThemeProvider>
        </main>
    );
}

export default Layout;