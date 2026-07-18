# Callout type charset

> [!custom-type] arbitrary type
> body

> [!Custom_Type-2] underscore and digits and case
> body

> [!NOTE] uppercase builtin
> body

> [!123] digits only
> body

> [!] empty type must stay a plain quote

> [!unclosed head falls back to plain quote
> second line

> [!two words] space in type is a plain quote

> [! note] leading space inside bracket

> [!注意] unicode type

   > [!three-indent] three spaces is still a quote

    > [!four-indent] four spaces is indented code, not a quote

> [!fence-host]
> ```
> > [!fake-in-fence] inside a fence inside a callout
> ```

```
> [!fake-in-plain-fence] not a callout
```
