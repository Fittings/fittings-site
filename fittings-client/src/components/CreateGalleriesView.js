import Galleries from '@/components/Galleries'

export default function createGalleriesView () {
  return {
    name: 'what-does-this-do',
    data () {
      return {
        msg: 'This is th!e galleries page!',
        name: 'Cameron'
      }
    },
    render (h) {
      return h(Galleries, { })
    }
  }
}
