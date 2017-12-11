declare namespace Impl {
    type ReactType = string | ComponentType<any>;
    type ComponentType<P = {}> = ComponentClass<P> | StatelessComponent<P>;

    type Key = string | number;
    type Ref<T> = string | { bivarianceHack(instance: T | null): any }["bivarianceHack"];

    interface Attributes {
        key?: Key;
    }
    interface ClassAttributes<T> extends Attributes {
        ref?: Ref<T>;
    }

    interface ReactElement<P> {
        type: string | ComponentClass<P> | SFC<P>;
        props: P;
        key: Key | null;
    }

    interface SFCElement<P> extends ReactElement<P> {
        type: SFC<P>;
    }

    type CElement<P, T extends Component<P>> = ComponentElement<P, T>;
    interface ComponentElement<P, T extends Component<P>> extends ReactElement<P> {
        type: ComponentClass<P>;
        ref?: Ref<T>;
    }

    type ReactText = string | number;
    type ReactChild = ReactElement<any> | ReactText;

    type ReactFragment = {} | Array<ReactChild | any[] | boolean>;
    type ReactNode = ReactChild | ReactFragment | boolean | null | undefined;

    function createElement<P>(
        type: SFC<P>,
        props?: Attributes & P,
        ...children: ReactNode[]): SFCElement<P>;
    function createElement<P, T extends Component<P>, C extends ComponentClass<P>>(
        type: ClassType<P, T, C>,
        props?: ClassAttributes<T> & P,
        ...children: ReactNode[]): CElement<P, T>;

    type ReactInstance = Component<any> | Element;

    interface Component<P = {}> { }
    class Component<P> {
        constructor(props?: P);
        render(): JSX.Element | null;
        props: Readonly<{ children?: ReactNode }> & Readonly<P>;
    }

    class PureComponent<P> extends Component<P> { }

    type SFC<P = {}> = StatelessComponent<P>;
    interface StatelessComponent<P = {}> {
        (props: P & { children?: ReactNode }): ReactElement<any> | null;
        propTypes?: ValidationMap<P>;
    }

    interface ComponentClass<P = {}> {
        new(props?: P): Component<P>;
        propTypes?: ValidationMap<P>;
    }

    type ClassType<P, T extends Component<P>, C extends ComponentClass<P>> =
        C &
        (new (props?: P) => T) &
        (new (props?: P) => { props: P });

    type Validator<T> = { bivarianceHack(object: T, key: string, componentName: string, ...rest: any[]): Error | null }["bivarianceHack"];

    interface Requireable<T> extends Validator<T> {
        isRequired: Validator<T>;
    }

    type ValidationMap<T> = {[K in keyof T]?: Validator<T> };
}
