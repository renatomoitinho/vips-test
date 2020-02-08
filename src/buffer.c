#include <stdio.h>
#include <stdlib.h>

#include <vips/vips.h>
/**
gcc -g -Wall buffer.c `pkg-config vips --cflags --libs`

*/
int
main( int argc, char **argv )
{
        char *buffer_in;
        size_t length_in;
        GError *error = NULL;
        VipsImage *image;
        char *buffer_out;
        size_t length_out;

        if( VIPS_INIT( argv[0] ) )
                vips_error_exit( NULL );

        /* Load source image into memory. This image will be in some format,
         * like JPEG.
         */
        if( !g_file_get_contents( argv[1], &buffer_in, &length_in, &error ) ) {
                fprintf( stderr, "unable to read file %s\n%s\n",
                        argv[1], error->message );
                g_error_free( error );
                exit( 1 );
        }

        /* Make a vips image from the memory buffer. This image will
         * decompress from buffer_in as required.
         */
        if( !(image = vips_image_new_from_buffer( buffer_in, length_in, NULL,
                "access", VIPS_ACCESS_SEQUENTIAL,
                NULL )) )
                vips_error_exit( NULL );

        /* Create an output memory buffer. Again, this buffer will contain an
         * image packed into some format, it will not be an array of pixel
         * values.
         */
        if( vips_image_write_to_buffer( image,
                ".jpg[Q=90,background=255]", (void **) &buffer_out, &length_out, NULL ) )
                vips_error_exit( NULL );

        if( !g_file_set_contents( argv[2], buffer_out, length_out, &error ) ) {
                fprintf( stderr, "unable to write file %s\n%s\n",
                        argv[1], error->message );
                g_error_free( error );
                exit( 1 );
        }

        g_object_unref( image );
        g_free( buffer_in );
        g_free( buffer_out );

        return( 0 );
}
