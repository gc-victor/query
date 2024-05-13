import { Button } from "@/pages/components/button";

export function Html404({ link }: { link: string }) {
    return (
        <div className="flex flex-col items-center justify-center py-12 text-center space-y-4">
            <p className="font-cal text-6xl">Page Not Found</p>
            <p className="text-lg ">The page you are looking for does not exist.</p>

            <Button tag="a" variant="md" href={link}>
                Go Home
            </Button>
        </div>
    );
}
