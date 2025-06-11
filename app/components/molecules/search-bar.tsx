import { useEffect, useId, useRef } from 'react';
import { Form } from 'react-router';

import { Input } from '~/components/ui/input';

const SearchBar = () => {
    const id = useId();
    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        const handleKeyDown = (event: KeyboardEvent) => {
            // Check for Cmd+K (Mac) or Ctrl+K (Windows/Linux)
            if ((event.metaKey || event.ctrlKey) && event.key === 'k') {
                event.preventDefault();
                inputRef.current?.focus();
            }
        };

        // Add event listener to document
        document.addEventListener('keydown', handleKeyDown);

        // Cleanup
        return () => {
            document.removeEventListener('keydown', handleKeyDown);
        };
    }, []);

    return (
        <Form className="relative p-1">
            <Input ref={inputRef} id={id} className="pe-11" placeholder="Filter by Key Name or Pattern" type="search" name="key" />
            <div className="text-muted-foreground pointer-events-none absolute inset-y-0 end-0 flex items-center justify-center pe-2">
                <button
                    type="submit"
                    className="text-muted-foreground/70 inline-flex h-5 max-h-full items-center rounded border px-1 font-[inherit] text-[0.625rem] font-medium"
                >
                    ⌘K
                </button>
            </div>
        </Form>
    );
};

export default SearchBar;
