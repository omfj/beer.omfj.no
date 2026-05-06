import { cva, type VariantProps } from 'class-variance-authority';

export const buttonStyle = cva(
	'text-xl flex items-center justify-center font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-50',
	{
		variants: {
			variant: {
				default: 'bg-background-darker hover:bg-background-darkest text-foreground',
				destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
				outline: 'border-2 border-input bg-background text-foreground hover:bg-background-dark',
				ghost: 'bg-transparent text-foreground hover:bg-background-dark'
			},
			size: {
				default: 'h-14 px-4',
				sm: 'h-10 px-3',
				lg: 'h-18 px-8'
			},
			link: {
				true: 'hover:underline',
				false: ''
			}
		},
		defaultVariants: {
			variant: 'default',
			size: 'default',
			link: false
		}
	}
);

export type ButtonStyleProps = VariantProps<typeof buttonStyle>;
